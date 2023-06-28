import { getAllChildrenInSelection } from "./getAllChildrenInSelection";

function keyPresshandler(event: KeyboardEvent) {
  if (!event || !event?.target) return
  let editableDiv = document.getElementById('wysiwyg') //event.target
  if (!editableDiv) return
  var keyCode = event.keyCode || event.which;
  if (keyCode === 13) {
    // Enter key
    event.preventDefault();
    let paragraphs: HTMLCollectionOf<HTMLParagraphElement> = editableDiv.getElementsByTagName("p");
    let lastParagraph = paragraphs[paragraphs.length - 1];

    if (lastParagraph && (lastParagraph?.textContent ?? '').trim() === "") {
      lastParagraph.remove();
    }
    
    // TODO could be inserted in middle
    const sel = getAllChildrenInSelection('wysiwyg')

    let newParagraph = document.createElement("p");
    newParagraph.contentEditable = 'true';
    newParagraph.onkeypress = keyPresshandler
    // newParagraph.setAttribute('style', 'margin: 0; height: fit-content;')
    editableDiv.appendChild(newParagraph);
    newParagraph.focus()
  } else if (keyCode === 8) {
    // Backspace 
    // @ts-ignore
    if (event.target.innerText === "") {
      // get all items in selection and remove from document
      console.log('empty, TODO remve') // See getAllChildrenInSelection
      //editableDiv.removeChild(event.)
    } 
  }
}

export {
  keyPresshandler
} 
