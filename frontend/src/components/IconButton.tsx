import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";

interface IconButtonProps {
  label: string;
  icon: LucideIcon;
  onClick?: () => void;
  active?: boolean;
  disabled?: boolean;
  size?: "sm" | "md" | "lg";
  children?: ReactNode;
}

export function IconButton({
  label,
  icon: Icon,
  onClick,
  active = false,
  disabled = false,
  size = "md",
  children,
}: IconButtonProps) {
  return (
    <button
      type="button"
      className={`icon-button icon-button--${size}${active ? " is-active" : ""}`}
      aria-label={label}
      aria-pressed={active ? true : undefined}
      disabled={disabled}
      onClick={onClick}
      title={label}
    >
      <Icon aria-hidden="true" size={size === "sm" ? 15 : size === "lg" ? 20 : 17} strokeWidth={1.8} />
      {children ? <span className="icon-button__label">{children}</span> : null}
    </button>
  );
}
