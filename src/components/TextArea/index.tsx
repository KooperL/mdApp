import { useContext, onCleanup, onMount, JSXElement } from 'solid-js';
import { useKeyboardContext } from '../../context/keyboardShortcuts';

interface TextAreaProps {
  id: string;
  children: JSXElement;
}

function TextArea({ id, children }: TextAreaProps) {
  const context = useKeyboardContext();
  if (!context) return <></>
  const { undo, redo, registerTextArea, applyChange } = context

  onMount(() => {
    registerTextArea(id, '');
  });

  let typingTimer: NodeJS.Timeout | null = null;

  const handleInput = (event: InputEvent) => {
    const textArea = event.target as HTMLTextAreaElement;
    const newValue = textArea.value;

    if (typingTimer) {
      clearTimeout(typingTimer);
    }

    typingTimer = setTimeout(() => {
      applyChange(id, newValue);
    }, 2000);
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.keyCode === 32) { // Pressed space
      const textArea = event.target as HTMLTextAreaElement;
      const newValue = textArea.value;

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

  window.addEventListener('keydown', handleKeyDown);

  onCleanup(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });

  return (
    <div
      contenteditable={true}
      id={id}
      onInput={handleInput}
      onKeyDown={handleKeyDown}
    >
      {children}
    </div>
  );
}

export default TextArea;
