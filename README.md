# Frontend
 - Leveraging HTML's contenteditable textarea to have native keyboard shortcuts for simple tasks (undo, ctrl + <arrow keys>)
 - Will post async callback to listener and update textarea with response XML

# Backend
- Function here will receive string, command and range
- String will be tokenized and then serialized into a vec of structs
- Meta tags/tokens will be removed here so range can index vec directly
- Vec will be deserialized into tree/string (challenges here)
- Function will return string of XML

# Integration
 - Tauri
 - WASM
