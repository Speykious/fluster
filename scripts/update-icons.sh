#!/bin/bash

# Removes unneeded glyphs from the Phosphor Icons file.
# The font has hundreds of glyphs, and we only need a few of them.
# Source: https://stackoverflow.com/questions/64614572/creating-a-material-icons-subset
#         Modified to adapt to Phosphor Icons.

# Dependencies: echo printf sed wget jq fonttools

# The icons to keep in the font
ICONS_FILL="browser eye eye-slash key"
ICONS_BOLD="control check"

FONT_FOLDER=assets/fonts      # Final folder location
FONT_FOLDER_TEMP=assets/.temp # Temp. folder location, used for files that should be ignored by git

PHOSPHOR_BASE_URL=https://raw.githubusercontent.com/phosphor-icons/web/master/src

function kebab_pascal () {
	echo "$1" | sed -r 's/(^|-)(.)/\U\2/g'
}

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
	echo "Removes unneeded glyphs from the Phosphor Icons file."
	echo
	echo "usage: $0 [-f | -h | --help]"
	echo
	echo "       -h | --help : Display this message."
	echo "       -f          : Force downloading of Phosphor files even if they already exist."
	exit
fi

# A file with conversions between icon names and codepoints and other information
if [ ! -f "$FONT_FOLDER_TEMP/Phosphor.json" ] || [ "$1" = "-f" ]; then
	PHOSPHOR_JSON_SRC="$PHOSPHOR_BASE_URL/Phosphor.json"
	wget -N -q --show-progress "$PHOSPHOR_JSON_SRC" -P "$FONT_FOLDER_TEMP/" >/dev/stderr
fi

function trim_phosphor_font () {
	FONT_TYPE="$1"
	ICONS="$2"

	# Download source of the Phosphor Icons font file
	# Attach `.ttf` for the font, or `.codepoints` for the codepoints
	FONT_TYPE_PASCALCASE="$(kebab_pascal "$FONT_TYPE")"
	FONT_FILENAME="Phosphor-$FONT_TYPE_PASCALCASE.ttf"
	if [ ! -f "$FONT_FOLDER_TEMP/$FONT_FILENAME" ] || [ "$1" = "-f" ]; then
			FONT_SRC="$PHOSPHOR_BASE_URL/$FONT_TYPE/$FONT_FILENAME"
			wget -N -q --show-progress "$FONT_SRC" -P "$FONT_FOLDER_TEMP/" >/dev/stderr
	fi

	echo >/dev/stderr
	echo "$FONT_TYPE - Generating codepoints for font type" >/dev/stderr
	CODEPOINTS_QUERY="[.iconSets.[] | select(.metadata.name == \"$FONT_TYPE_PASCALCASE\") | .selection.[] | { key: .name, value: .code }] | from_entries"
	CODEPOINTS_FILE="$FONT_FOLDER_TEMP/Phosphor-$FONT_TYPE_PASCALCASE.codepoints.json"
	jq "$CODEPOINTS_QUERY" "$FONT_FOLDER_TEMP/Phosphor.json" > "$CODEPOINTS_FILE"

	echo "$FONT_TYPE - Get codepoints of specified icons" >/dev/stderr
	CODEPOINTS=""
	for ICON in $ICONS; do
		CODEPOINT_DEC=$(jq ".\"$ICON-$FONT_TYPE\"" "$CODEPOINTS_FILE")
		CODEPOINT=$(printf '%x' "$CODEPOINT_DEC")

		# function's output
		echo -e "$(kebab_pascal "$ICON")=0x$CODEPOINT,"

		if [[ -z "$CODEPOINTS" ]]
		then CODEPOINTS="$CODEPOINT"
		else CODEPOINTS="$CODEPOINTS,$CODEPOINT"
		fi
	done

	echo "$FONT_TYPE - Trimming font..." >/dev/stderr
	fonttools subset \
	    "$FONT_FOLDER_TEMP/$FONT_FILENAME" \
	    --unicodes="$CODEPOINTS" \
	    --no-layout-closure \
	    --output-file="$FONT_FOLDER/Phosphor-$FONT_TYPE_PASCALCASE.trimmed.ttf"
}

ENUM_FIELDS_FILL="$(trim_phosphor_font fill "$ICONS_FILL")"
ENUM_FIELDS_BOLD="$(trim_phosphor_font bold "$ICONS_BOLD")"

ENUM_FIELDS=$(echo "$ENUM_FIELDS_FILL" "$ENUM_FIELDS_BOLD" | sort | uniq)

echo >/dev/stderr
echo "Generating enum" >/dev/stderr
echo >/dev/stderr
echo "pub enum PhosphorIcon {"
for field in $ENUM_FIELDS; do
	echo -e "\t$field" | sed 's/=/ = /g'
done
echo -n "}"
echo >/dev/stderr
echo >/dev/stderr
echo "Done" >/dev/stderr
