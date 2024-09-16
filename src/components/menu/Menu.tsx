import './style.css';
import Button from '../../ui/button/Button.tsx';

export default function Menu() {
  const toggle = () => {
    console.log('Toggle');
  };
  return (
    <div>
      <Button onlyIcon={true} onClick={toggle} icon="hugeicons:dashboard-square-02" />
    </div>
  );
}
