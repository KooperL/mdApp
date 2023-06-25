import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";

function test(event: KeyboardEvent) {
  if (!event) return
  let editableDiv = document.getElementById('wysiwyg') //event.target
  if (!editableDiv) return
  var keyCode = event.keyCode || event.which;
  if (keyCode === 13) {
    event.preventDefault();
    // @ts-ignore
    var paragraphs: HTMLCollectionOf<HTMLParagraphElement> = editableDiv.getElementsByTagName("p");
    var lastParagraph = paragraphs[paragraphs.length - 1];

    // @ts-ignore
    if (lastParagraph && lastParagraph.textContent.trim() === "") {
      lastParagraph.remove();
    }

    var newParagraph = document.createElement("p");
    // @ts-ignore
    newParagraph.contenteditable = true;
    newParagraph.onkeypress = test
    editableDiv.appendChild(newParagraph);
    newParagraph.focus()
  } else if (keyCode === 8) {
    // @ts-ignore
    if (event.target.innerHTML === "") {
      console.log('empty')
    } 
  }
}


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
        <div
          id="wysiwyg"
          onKeyDown={test}
            contenteditable={true}
        >
          <p 
            contenteditable={true}
          onKeyDown={test}
          ></p>
        </div>
      </form>
    </div>
  );
}

export default App;
