function contentEditableFocus(event: any) {
  var innerParagraph = document.createElement("p");
  this.appendChild(innerParagraph);

  // Set the focus on the nested <p> element for text input
  innerParagraph.focus();
}

export {}
