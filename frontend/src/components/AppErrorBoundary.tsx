import { Component, type ErrorInfo, type ReactNode } from "react";
import { AlertTriangle, RefreshCw } from "lucide-react";

interface AppErrorBoundaryProps {
  children: ReactNode;
}

interface AppErrorBoundaryState {
  hasError: boolean;
}

export class AppErrorBoundary extends Component<AppErrorBoundaryProps, AppErrorBoundaryState> {
  state: AppErrorBoundaryState = { hasError: false };

  static getDerivedStateFromError(): AppErrorBoundaryState {
    return { hasError: true };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error("PlayBack UI failed to render", error, info);
  }

  render() {
    if (!this.state.hasError) {
      return this.props.children;
    }

    return (
      <main className="recovery-screen" role="alert">
        <div className="recovery-screen__card">
          <span className="recovery-screen__icon" aria-hidden="true">
            <AlertTriangle size={24} strokeWidth={1.6} />
          </span>
          <p className="recovery-screen__eyebrow">PlayBack needs a reset</p>
          <h1>The player window lost its interface.</h1>
          <p className="recovery-screen__copy">
            Your media and settings are safe. Reload the window to return to the library.
          </p>
          <button type="button" className="primary-button" onClick={() => window.location.reload()}>
            <RefreshCw aria-hidden="true" size={16} />
            <span>Reload PlayBack</span>
          </button>
          <span className="recovery-screen__hint">If this repeats, restart the desktop app.</span>
        </div>
      </main>
    );
  }
}
