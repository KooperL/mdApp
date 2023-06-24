import { invoke } from "@tauri-apps/api/tauri";

async function updateSelection(content_id: string, action: string) {
  
  const selection: Selection | null = window.getSelection();
  if (!selection) return null
  const anchor = selection.anchorNode?.parentNode // Must be p tag, deeply nested styles could destory existing logic
  const anchorOffset = selection.anchorOffset
  const focus = selection.focusNode?.parentNode
  const focusOffset = selection.focusOffset

  // right to left vs left to right???

  const wysiwyg = document.getElementById(content_id)
  // Must be p, must contain both nodes: validation acativity
  // parentNode.parentNode

  if (!wysiwyg) return null

  let isRelevant = false
  const children = wysiwyg.children
  let iterator = 0
  let shouldIterate = true
  while (shouldIterate) {
    const element = children[iterator]
    if (element === anchor) {
      let isRelevant = true 
    }

    if (isRelevant) {
      // action
      const newHTML: string = await invoke("process_styling", { html: element.innerHTML, begin: anchorOffset, end: focusOffset, transformation: action })
      element.innerHTML = newHTML
    }

    if (element === focus) {
      shouldIterate = false
      isRelevant = false
    }
    iterator ++
  }


}

// user highlights
// user presses button
// handler function fires, b (example) passed as arg
// getRangeSelection is called, rawHTML and range indexes of text are returned
// Passed to web worker
// updateSelection is called

export {
  updateSelection,
}
