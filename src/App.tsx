import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";
import { getAllChildrenInSelection } from "./utils/getAllChildrenInSelection";
import { keyPresshandler } from "./utils/keyboardEventHandler";


function App() {
  const [str, setStr] = createSignal("");
  const contentID = "wysiwyg"

  return (
    <div class="container">
      <form
        class="row"
        onSubmit={async (e) => {
          e.preventDefault();
          if (!document.querySelector(`#${contentID}`)) throw new Error()
        }}
      >
        <div class="toolbar">
          <button onclick={() => {
            updateSelection(contentID, "bold")
          }}
        >
          <b>Bold</b>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "emphasise")
          }}
        >
          <i>Italics</i>
        </button>
        </div>
        <div id="content-container">
          <div
            id={contentID}
            contenteditable={true}
          >
            <p 
              contenteditable={true}
              onKeyDown={keyPresshandler}
            >&#8203;</p>
          </div>
        </div>
      </form>
    </div>
  );
}

// Spaces: \u{3164} \u{200b}

export default App;
