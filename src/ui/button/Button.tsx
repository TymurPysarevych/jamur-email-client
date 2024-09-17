import './style.scss';
import { Icon } from '@iconify/react';
import { useEffect, useRef } from 'react';
import autoAnimate from '@formkit/auto-animate';

interface ButtonProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  onClick: () => void;
  text?: string;
  icon?: string;
  onlyIcon?: boolean;
  disabled?: boolean;
}

const ICON_SIZE = '24px';

export default function Button({ onClick, text, icon, onlyIcon, disabled }: ButtonProps) {
  const parent = useRef(null);
  useEffect(() => {
    parent.current && autoAnimate(parent.current, { duration: 200 });
  }, [parent]);

  const onClickProxy = () => {
    if (disabled) {
      return;
    }
    onClick();
  };

  const buttonTemplate = () => {
    const textAndIconPresent = text && icon;
    const className = disabled ? 'button-disabled' : 'button';
    return (
      <div className={className} onClick={onClickProxy}>
        {icon && <Icon width={ICON_SIZE} icon={icon} />}
        {text && <span style={{ marginLeft: `${textAndIconPresent ? '8px' : '0'}` }}>{text}</span>}
      </div>
    );
  };

  if (onlyIcon && icon) {
    return <Icon icon={icon} width={ICON_SIZE} onClick={onClick} />;
  }

  return (
    <div ref={parent}>
      {disabled && buttonTemplate()}
      {!disabled && buttonTemplate()}
    </div>
  );
}
