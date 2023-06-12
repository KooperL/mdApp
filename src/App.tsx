import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [str, setStr] = createSignal("");

  async function process() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setStr(await invoke("process_arg", { name: str() }));
  }

  return (
    <div class="container">
      <h1>mdApp</h1>
      <form
        class="row"
        onSubmit={async (e) => {
          e.preventDefault();
          await process();
          if (!document.querySelector('#greet-input')) throw new Error()
          document.querySelector('#greet-input')!.innerHTML = str.toString()
        }}
      >
        <textarea contenteditable={true}
          id="greet-input"
          onChange={(e) => setStr(e.currentTarget.value)}
          placeholder="Text"
          />
        <button type="submit">process text</button>
      </form>
    </div>
  );
}

export default App;
