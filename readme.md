# WasiP3 multi-component demo

1. backend component with oxiwhisper = "0.1.1", input: audio file, output: transcription text
2. frontend component: $ curl http://localhost:8000/ 
3. middleware component: frontend -> middleware component[ some preocess ] -> backend component

add a testdata directory at root
with files 
1. p3-multi-comps-demo/testdata/ggml-tiny.bin
2. p3-multi-comps-demo/testdata/jfk.wav 
