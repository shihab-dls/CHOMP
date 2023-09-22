mkdir test_data/
for FILE_NAME in $(find /dls/labxchem/data/ -mindepth 5 -maxdepth 5 -path */processing/database/soakDBDataFile.sqlite ! -path */20*)
do
    VISIT=$(cut -d / -f 6 <<< $FILE_NAME)
    cp "$FILE_NAME" "test_data/$VISIT.sqlite"
done
