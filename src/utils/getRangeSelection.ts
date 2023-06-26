import { invoke } from "@tauri-apps/api/tauri";



function getSelectionCharacterOffsetWithin(element) {
    var start = 0;
    var end = 0;
    var doc = element.ownerDocument || element.document;
    var win = doc.defaultView || doc.parentWindow;
    var sel;
    if (typeof win.getSelection != "undefined") {
        sel = win.getSelection();
        if (sel.rangeCount > 0) {
            var range = win.getSelection().getRangeAt(0);
            var preCaretRange = range.cloneRange();
            preCaretRange.selectNodeContents(element);
            preCaretRange.setEnd(range.startContainer, range.startOffset);
            start = preCaretRange.toString().length;
            preCaretRange.setEnd(range.endContainer, range.endOffset);
            end = preCaretRange.toString().length;
        }
    } else if ( (sel = doc.selection) && sel.type != "Control") {
        var textRange = sel.createRange();
        var preCaretTextRange = doc.body.createTextRange();
        preCaretTextRange.moveToElementText(element);
        preCaretTextRange.setEndPoint("EndToStart", textRange);
        start = preCaretTextRange.text.length;
        preCaretTextRange.setEndPoint("EndToEnd", textRange);
        end = preCaretTextRange.text.length;
    }
    return { start: start, end: end };
}

async function updateSelection(content_id: string, action: string) {
  console.log('called')
  const selection: Selection | null = window.getSelection();
  if (!selection) return null
  // const anchor = selection.anchorNode?.parentNode // Must be p tag, deeply nested styles could destory existing logic
  // focusOffset
  // const focus = selection.focusNode?.parentNode
  // anchorOffset

  const {start, end} = getSelectionCharacterOffsetWithin( document.getElementById(content_id) );
  // right to left vs left to right???

  const wysiwyg = document.getElementById(content_id)
  // Must be p, must contain both nodes: validation acativity

  if (!wysiwyg) return null

  console.log('begin loop')
  let isRelevant = false
  const children = wysiwyg.children
  let iterator = 0
  let shouldIterate = true
  let rollingTicker = 0
  while (shouldIterate) {
    const element = children[iterator]
    console.log(element, children)

    // if (element.isSameNode(anchor)) {
    if (start > rollingTicker) {
      console.log('same node found')
      isRelevant = true 
    }

    console.log('loop')
    if (isRelevant) {
      console.log('loop is relevant')
      // console.log({ html: element.outerHTML, begin: anchorOffset, end: focusOffset, transformation: action })
      // const newHTML: string = await invoke("process_styling", { html: element.outerHTML, begin: anchorOffset, end: focusOffset, transformation: action })
      
      console.log(element.outerHTML, element.outerHTML.length, element.textContent, element.textContent.length, start, end, rollingTicker)
      // const newHTML: string = await invoke("process_styling", { html: element.outerHTML, begin: start - rollingTicker, end: end - rollingTicker, transformation: action })
      
      // console.log(newHTML)
      // element.outerHTML = newHTML
    }

    console.log(rollingTicker, end)
    // if (element.isSameNode(focus)) {
    rollingTicker += element.textContent?.length ?? 0
    if (end <= rollingTicker) {
      shouldIterate = false
      isRelevant = false
    }
    iterator ++
  }

  // Nah man, rollingVal is increased at start. if start caret is before rollingVal - node len and rollingVal, apply. if end caret is after rolling val, iterate

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
