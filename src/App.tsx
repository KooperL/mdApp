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
        <div
          id="wysiwyg"
          contenteditable={true}
          onKeyDown={function(event) {
  var contentDiv = event.target;
  var content = contentDiv.innerHTML;
  var newContent = "";

  if (content === "") {
    // Case a): No content in the editor
    contentDiv.innerHTML = "";
    return;
  }

  var paragraphs = content.split("<br>");
  paragraphs.forEach(function(paragraph) {
    if (paragraph.startsWith("<h") || paragraph.startsWith("</h")) {
      // Exclude header tags
      newContent += paragraph;
    } else {
      // Convert other content into paragraphs
      newContent += "<p>" + paragraph + "</p>";
    }
    newContent += "<br>";
  });

  // Remove the extra line break at the end
  newContent = newContent.slice(0, -4);

  // Update the content with modified paragraphs
  contentDiv.innerHTML = newContent;
}}
        >
          <p 
          ></p>
        </div>
      </form>
    </div>
  );
}

export default App;
