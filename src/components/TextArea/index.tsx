import { useContext, onCleanup, onMount, JSXElement } from 'solid-js';
import { useKeyboardContext } from '../../context/keyboardShortcuts';
import { handleKeyDown } from './listeners';

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
  const keyDownHandler = (e: KeyboardEvent) => handleKeyDown(
    e, typingTimer, id, applyChange, undo, redo
  )

  const handleInput = (event: InputEvent) => {
    const textArea = event.target as HTMLTextAreaElement;
    const newValue = textArea.innerHTML;
    if (typingTimer) {
      clearTimeout(typingTimer);
    }

    typingTimer = setTimeout(() => {
      applyChange(id, newValue);
    }, 2000);
  };

  window.addEventListener('keydown', keyDownHandler);

  onCleanup(() => {
    window.removeEventListener('keydown', keyDownHandler);
  });

  return (
    <div
      contenteditable={true}
      id={id}
      onInput={handleInput}
      onKeyDown={keyDownHandler}
    >
      <p 
      >
        &#8203;{children}
      </p>
    </div>
  );
}

export default TextArea;
