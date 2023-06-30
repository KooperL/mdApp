// @ts-ignore
function getCurrentParagraph(range) {
  let container = range.commonAncestorContainer;

  // If the current container is a text node, get its parent element
  if (container.nodeType === Node.TEXT_NODE) {
    container = container.parentNode;
  }

  // If the current container is not a <p> element, find the closest ancestor <p> element
  while (container && container.nodeName !== "P") {
    container = container.parentNode;
  }

  // If no <p> element is found, use the contenteditable div as the current container
  if (!container) {
    container = document.getElementById("editableDiv");
  }

  return container;
}

function contentEditableReturn(event: any) {
  let keyCode = event.keyCode || event.which;

  if (keyCode === 13) { // Check for the Enter key
    event.preventDefault(); // Prevent the default behavior of the Enter key

    let selection = window.getSelection();
    if (!selection) return
    let range = selection.getRangeAt(0);

    // Create a new <p> element
    let newParagraph = document.createElement("p");
    newParagraph.textContent = "\u200B"; // Add a zero-width space as placeholder text

    // Insert the new <p> element after the current <p> element or at the end of the div
    let currentParagraph = getCurrentParagraph(range);
    if (currentParagraph.nextSibling && currentParagraph.nextSibling.nodeName === "p") {
      currentParagraph.parentNode.insertBefore(newParagraph, currentParagraph.nextSibling);
    } else {
      currentParagraph.parentNode.appendChild(newParagraph);
    }

    // Move the caret to the newly created <p> element
    range.setStart(newParagraph, 0);
    range.collapse(true);
    selection.removeAllRanges();
    selection.addRange(range);
  } else if (keyCode === 8) { // Check for the Backspace key
    let selection = window.getSelection();
    if (!selection) return
    let range = selection.getRangeAt(0);

    // Get the current <p> element
    var currentParagraph = getCurrentParagraph(range);

    if (currentParagraph.textContent === "" && currentParagraph.previousSibling) {
      event.preventDefault(); // Prevent the default behavior of the Backspace key

      // Move the caret to the end of the previous <p> element
      let previousParagraph = currentParagraph.previousSibling;
      range.setStart(previousParagraph, previousParagraph.childNodes.length);
      range.collapse(true);
      selection.removeAllRanges();
      selection.addRange(range);

      // Remove the empty <p> element
      currentParagraph.parentNode.removeChild(currentParagraph);
    }
  }
};

export {
}
