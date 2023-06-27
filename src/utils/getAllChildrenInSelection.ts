function getAllChildrenInSelection(content_id: string) {
  let listOfElements: Element[] = new Array()
  const wysiwyg = document.getElementById(content_id)
  const selection: Selection | null = window.getSelection();
  if (!selection || !wysiwyg) return null
  
  const { children } = wysiwyg
  let iterator = 0
  let shouldIterate = false
  let isRelevant = false
    const {
      focusNode,
      focusOffset,
      anchorNode,
      anchorOffset 
    } = selection

  while (shouldIterate) {
    const element = children?.[iterator] ?? null 
    if (!element) return listOfElements
    if (element.isSameNode(anchorNode)) {
      isRelevant = true
    }

    if (isRelevant) {
      listOfElements.push(element)
    }
    
    if (element.isSameNode(focusNode)) {
      shouldIterate = false
    }
    iterator ++
  }

  return listOfElements 
}


export {
  getAllChildrenInSelection
}
