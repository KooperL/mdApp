import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [str, setStr] = createSignal("");

  async function updateStyling() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setStr(await invoke("process_styling", { name: str() }));
  }

  return (
    <div class="container">
      <h1>mdApp</h1>
      <form
        class="row"
        onSubmit={async (e) => {
          e.preventDefault();
          await updateStyling();
          if (!document.querySelector('#wysiwyg')) throw new Error()
          document.querySelector('#wysiwyg')!.innerHTML = str.toString()
        }}
      >
        <textarea contenteditable={true}
          id="wysiwyg"
          onChange={(e) => setStr(e.currentTarget.value)}
          placeholder="Text"
          />
        <button type="submit">process text</button>
      </form>
    </div>
  );
}

export default App;
