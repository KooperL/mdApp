import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";

function keyPresshandler(event: KeyboardEvent) {
  if (!event || !event?.target) return
  let editableDiv = document.getElementById('wysiwyg') //event.target
  if (!editableDiv) return
  var keyCode = event.keyCode || event.which;
  if (keyCode === 13) {
    event.preventDefault();
    var paragraphs: HTMLCollectionOf<HTMLParagraphElement> = editableDiv.getElementsByTagName("p");
    var lastParagraph = paragraphs[paragraphs.length - 1];

    if (lastParagraph && (lastParagraph?.textContent ?? '').trim() === "") {
      lastParagraph.remove();
    }
    
    // TODO could be inserted in middle

    let newParagraph = document.createElement("p");
    newParagraph.contentEditable = 'true';
    newParagraph.onkeypress = keyPresshandler
    // newParagraph.setAttribute('style', 'margin: 0; height: fit-content;')
    editableDiv.appendChild(newParagraph);
    newParagraph.focus()
  } else if (keyCode === 8) {
    // @ts-ignore
    if (event.target.innerText === "") {
      // get all items in selection and remove from document
      console.log('empty, TODO remve')
      //editableDiv.removeChild(event.)
    } 
  }
}


function App() {
  const [str, setStr] = createSignal("");

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
            contenteditable={true}
        >
          <p 
            contenteditable={true}
            onKeyDown={keyPresshandler}
          >‎</p>
        </div>
      </form>
    </div>
  );
}

export default App;
