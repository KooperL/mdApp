// Too much freedom, can bold code tags etc...

import React, { useRef } from 'react';

function App() {
  const contentEditableRef = useRef(null);

  const handleBoldClick = () => {
    document.execCommand('bold', false, null);
  };

  const handleUnderlineClick = () => {
    document.execCommand('underline', false, null);
  };

  const handleIncreaseFontSizeClick = () => {
    document.execCommand('fontSize', false, '2');
  };

  const handleInsertOrderedListClick = () => {
    document.execCommand('insertOrderedList', false, null);
  };

  const handleInsertUnorderedListClick = () => {
    document.execCommand('insertUnorderedList', false, null);
  };

  const handleIndentClick = () => {
    document.execCommand('indent', false, null);
  };

  const handleOutdentClick = () => {
    document.execCommand('outdent', false, null);
  };

  const handleAddTableClick = () => {
    const table = document.createElement('table');
    const tbody = document.createElement('tbody');
  
    for (let i = 0; i < 2; i++) {
      const row = document.createElement('tr');
      for (let j = 0; j < 2; j++) {
        const cell = document.createElement('td');
        cell.textContent = 'Cell';
        row.appendChild(cell);
      }
      tbody.appendChild(row);
    }
  
    table.appendChild(tbody);
  
    document.execCommand('insertHTML', false, table.outerHTML);
  };
  
  const handleInlineCodeClick = () => {
    const selection = window.getSelection();
    if (selection.rangeCount === 0) return;
  
    const range = selection.getRangeAt(0);
    const codeElement = document.createElement('code');
    codeElement.appendChild(range.extractContents());
    range.insertNode(codeElement);
  };
  
  const handleBlockCodeClick = () => {
    const selection = window.getSelection();
    if (selection.rangeCount === 0) return;
  
    const range = selection.getRangeAt(0);
    const preElement = document.createElement('pre');
    preElement.appendChild(range.extractContents());
    range.insertNode(preElement);
  };

  const handleInsertLinkClick = () => {
    const url = 'https://www.google.com';
    const selection = window.getSelection();
    const range = selection.getRangeAt(0);

    const link = document.createElement('a');
    link.href = url;
    link.textContent = selection.toString();

    range.deleteContents();
    range.insertNode(link);
  };

  return (
    <div>
      <div>
        <button onClick={handleBoldClick}>Bold</button>
        <button onClick={handleUnderlineClick}>Underline</button>
        <button onClick={handleIncreaseFontSizeClick}>Increase Font Size</button>
        <button onClick={handleInsertOrderedListClick}>Ordered List</button>
        <button onClick={handleInsertUnorderedListClick}>Unordered List</button>
        <button onClick={handleIndentClick}>Indent</button>
        <button onClick={handleOutdentClick}>Outdent</button>
        <button onClick={handleInsertLinkClick}>Insert Link</button>
        <button onClick={handleInlineCodeClick}>Inline code</button>
        <button onClick={handleBlockCodeClick}>Code block</button>
        <button onClick={handleAddTableClick}>Blank table</button>
      </div>
      <div
        contentEditable
        ref={contentEditableRef}
        style={{ border: '1px solid black', minHeight: '100px', padding: '10px' }}
      ></div>
    </div>
  );
}

export default App;

