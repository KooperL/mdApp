export default {}

interface SelectionInformation {
  range: Selection
  nodeCollection: Node[]
} 

function getRangeSelection(content_id: string): null | SelectionInformation {
  const selection: Selection | null = window.getSelection();
  if (!selection) return null
  const anchor = selection.anchorNode
  const anchorOffset = selection.anchorOffset
  const focus = selection.focusNode
  const focusOffset = selection.focusOffset

  // right to left vs left to right???

  const wysiwyg = document.getElementById(content_id)
  // Must be p, must contain both nodes: validation acativity

  if (!wysiwyg) return null

  let isInsideSelection = false
  let nodeCollection: Node[] = new Array() 
  wysiwyg.childNodes.forEach((element, elementIndex) => {
    if (element === anchor) {
      let isInsideSelection = true 
    }
    if (isInsideSelection === true) nodeCollection.push(element)
    if (element === focus) {
      let isInsideSelection = false 
    }
  });

  const returnVal: SelectionInformation = {
    nodeCollection,
    range: selection
  }
  return returnVal 
}

function updateSelection(selection: SelectionInformation) {
  const rangeAnchor = selection.range.getRangeAt(0)
  rangeAnchor.deleteContents()
  for (let i = 0; i < selection.nodeCollection.length; i++) {
    rangeAnchor.insertNode(selection.nodeCollection[i])
  } 
}

// user highlights
// user presses button
// handler function fires, b (example) passed as arg
// getRangeSelection is called, rawHTML and range indexes of text are returned
// Passed to web worker
// updateSelection is called

export {
  getRangeSelection,
  updateSelection
}
