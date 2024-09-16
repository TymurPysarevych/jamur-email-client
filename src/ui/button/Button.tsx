import './style.css';
import { Icon } from '@iconify/react';

interface ButtonProps {
  onClick: () => void;
  text?: string;
  icon?: string;
  onlyIcon?: boolean;
}

const ICON_SIZE = '24px';

export default function Button({ text, onClick, icon, onlyIcon }: ButtonProps) {
  if (onlyIcon && icon) {
    return <Icon icon={icon} width={ICON_SIZE} onClick={onClick} />;
  }

  const textAndIconPresent = text && icon;
  return (
    <>
      <div className={'button'} onClick={onClick}>
        {icon && <Icon width={ICON_SIZE} icon={icon} />}
        {text && <span style={{ marginLeft: `${textAndIconPresent ? '8px' : '0'}` }}>{text}</span>}
      </div>
    </>
  );
}
