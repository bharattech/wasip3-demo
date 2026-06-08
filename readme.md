# WasiP3 multi-component demo

1. Backend Component with crate oxiwhisper = "0.1.1", Input: audio file, Output: transcription text
2. Frontend Component: $ curl http://localhost:8000/ 
3. Middleware Component: Frontend -> Middleware component[ some process ] -> Backend component

Note: Add a testdata directory at root
with files 
1. p3-multi-comps-demo/testdata/ggml-tiny.bin
2. p3-multi-comps-demo/testdata/jfk.wav 
