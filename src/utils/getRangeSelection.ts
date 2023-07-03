import { invoke } from "@tauri-apps/api/tauri";


function getSelectionCharacterOffsetWithin(element: HTMLElement) {
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
    } else if (
      (sel = doc.selection) && sel.type != "Control")
    {
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

// TODO: If selection length is 0, apply transformation to whole word
async function updateSelection(content_id: string, action: string) {
  const wysiwyg = document.getElementById(content_id)
  const selection: Selection | null = window.getSelection();
  if (!selection) return null

  const {start, end} = getSelectionCharacterOffsetWithin(wysiwyg as HTMLElement);
  // TODO: right to left vs left to right???

  if (!wysiwyg) return

  let isRelevant = false
  const { children } = wysiwyg
  let iterator = 0
  let shouldIterate = true
  let rollingTicker = 0

  while (shouldIterate) {
    const element = children[iterator] ?? null
    if (!element) return
    let contentLength = element?.textContent?.length ?? 0

    let textBegin = rollingTicker 
    rollingTicker += contentLength
    let textEnd = rollingTicker

    let startCaretIsInThisElement = (
      start >= textBegin &&
      start < textEnd
    )

    let endCaretIsInThisElement = (
      end >= textBegin &&
      end <= textEnd
    )
    
    if (startCaretIsInThisElement || isRelevant) {
      isRelevant = true
      const args = {
        html: element.outerHTML,
        begin: Math.max(0, start - rollingTicker + contentLength),
        end: Math.max(Math.min(end - rollingTicker + contentLength, contentLength), 0),
        transformation: action
      }
      // console.log(args)
      const newHTML: string = await invoke("process_styling", args)
      
      console.log(newHTML)
      element.outerHTML = newHTML
    }

    if (endCaretIsInThisElement) {
      shouldIterate = false 
      isRelevant = false 
    }

    iterator ++
  }
  selection.removeAllRanges()
}

export {
  updateSelection,
}
