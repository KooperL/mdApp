import { Props as exportProps } from "../../components/toolbar/types"
import { Props as importProps} from './../../views/editor/types'

function toolbarController(props: importProps): exportProps {
  const { contentEditableRef } = props

  const handleBoldClick = () => {
    const selection = window.getSelection();
    if (selection.rangeCount === 0) return;
    const range = selection.getRangeAt(0);
    const parentElement = range.commonAncestorContainer.parentElement; // Check if the common ancestor is a code, pre, or table element

    // Check if the common ancestor is a code, pre, or table element
    if (parentElement.closest('code') || parentElement.closest('pre') || parentElement.closest('table')) {
      return;
    }

    document.execCommand('bold', false, null);
  };

  const handleUnderlineClick = () => {
    const selection = window.getSelection();
    if (selection.rangeCount === 0) return;
    const range = selection.getRangeAt(0);
    const parentElement = range.commonAncestorContainer.parentElement; // Check if the common ancestor is a code, pre, or table element

    // Check if the common ancestor is a code, pre, or table element
    if (parentElement.closest('code') || parentElement.closest('pre') || parentElement.closest('table')) {
      return;
    }

    document.execCommand('underline', false, null);
  };

  const handleIncreaseFontSizeClick = () => {
    const selection = window.getSelection();
    if (selection.rangeCount === 0) return;
    const range = selection.getRangeAt(0);
    const parentElement = range.commonAncestorContainer.parentElement; // Check if the common ancestor is a code, pre, or table element

    // Check if the common ancestor is a code, pre, or table element
    if (parentElement.closest('code') || parentElement.closest('pre') || parentElement.closest('table')) {
      return;
    }

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

  return {
    handleBoldClick,
    handleIndentClick,
    handleOutdentClick,
    handleAddTableClick,
    handleBlockCodeClick,
    handleUnderlineClick,
    handleInlineCodeClick,
    handleInsertLinkClick,
    handleIncreaseFontSizeClick,
    handleInsertOrderedListClick,
    handleInsertUnorderedListClick
  }
}

export default toolbarController
