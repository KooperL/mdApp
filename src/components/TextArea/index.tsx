import { useContext, onCleanup, onMount } from 'solid-js';
import { KeyboardContext } from '../../context/keyboardShortcuts';

interface TextAreaProps {
  id: string;
  initialValue: string;
}

function TextArea({ id, initialValue }: TextAreaProps) {
  const { undo, redo, registerTextArea, applyChange } = useContext(KeyboardContext);

  // Register the text area when it mounts
  onMount(() => {
    registerTextArea(id, initialValue);
  });

  let typingTimer: NodeJS.Timeout | null = null;

  const handleInput = (event: InputEvent) => {
    const textArea = event.target as HTMLTextAreaElement;
    const newValue = textArea.value;

    // Clear the previous typing timer, if any
    if (typingTimer) {
      clearTimeout(typingTimer);
    }

    // Set a new typing timer for 2 seconds
    typingTimer = setTimeout(() => {
      // Apply the change to the undo/redo history after 2 seconds
      applyChange(id, newValue);
    }, 2000);
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    // Check if the spacebar key is pressed (key code 32)
    if (event.keyCode === 32) {
      const textArea = event.target as HTMLTextAreaElement;
      const newValue = textArea.value;

      // Clear the previous typing timer, if any
      if (typingTimer) {
        clearTimeout(typingTimer);
      }

      // Apply the change to the undo/redo history when spacebar is pressed
      applyChange(id, newValue);
    }

  // Add event listener for Control + Z and Control + Y (redo)
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

  // Add event listener when the component mounts
  window.addEventListener('keydown', handleKeyDown);

  // Cleanup event listener when the component unmounts
  onCleanup(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });
  return (
    <div>
      <textarea
        id={id}
        placeholder={initialValue}
        onInput={handleInput}
        onKeyDown={handleKeyDown}
      />
      <button onClick={() => undo(id)}>Undo (Ctrl + Z)</button>
      <button onClick={() => redo(id)}>Redo (Ctrl + Y)</button>
    </div>
  );
}

export default TextArea;
