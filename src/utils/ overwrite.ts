function formatElement (element: string, tagName: string) {
    const node = document.querySelector(element);
  if (!node) return
  const newElement = document.querySelector('<'+tagName+'/>').append(node.clone().get(0).childNodes);
  node.replaceWith(newElement);
}

export {
  formatElement
}