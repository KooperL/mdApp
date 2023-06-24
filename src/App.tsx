import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";

function App() {
  const [str, setStr] = createSignal("");

  async function updateStyling() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  }

  return (
    <div class="container">
      <h1>mdApp</h1>
      <form
        class="row"
        onSubmit={async (e) => {
          e.preventDefault();
          if (!document.querySelector('#wysiwyg')) throw new Error()
        }}
      >
        <div class="toolbar">
          <button onclick={() => {
            updateSelection("wysiwyg", "bold")
          }}
        >Bold</button>
        </div>
        <div contenteditable={true}
          id="wysiwyg"
        />
      </form>
    </div>
  );
}

export default App;
