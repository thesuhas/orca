for file in *.wast ;
do
    wasm-tools json-from-wast "$file" ;
    wasm-tools print "${file%.*}.0.wasm" -o "${file%.*}.wat";
done