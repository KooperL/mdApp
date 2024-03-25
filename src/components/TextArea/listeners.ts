  const handleKeyDown = (
    event: KeyboardEvent,
    typingTimer: NodeJS.Timeout | null,
    id: any,
    applyChange: (id: any, newValue: any) => void,
    undo: (id: any) => void,
    redo: (id: any) => void
  ) => {
    if (event.keyCode === 32) { // Pressed space
      const textArea = event.target as HTMLTextAreaElement;
      const newValue = textArea.innerHTML;

      if (typingTimer) {
        clearTimeout(typingTimer);
      }

      applyChange(id, newValue);
    }
    if (event.ctrlKey && event.key === 'z') {
      event.preventDefault();
      const currentTextArea = document.activeElement as HTMLTextAreaElement;
      const id = currentTextArea.id;
      undo(id);
    } else if (event.ctrlKey && event.key === 'y') {
      event.preventDefault();
      const currentTextArea = document.activeElement as HTMLTextAreaElement;
      const id = currentTextArea.id;
      redo(id);
    }
  };

  export {
    handleKeyDown
  }
