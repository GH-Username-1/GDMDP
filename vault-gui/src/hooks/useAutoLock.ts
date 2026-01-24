import { useEffect, useRef } from "react";

/**
 * Hook pour l'auto-lock après inactivité
 */
export function useAutoLock(
  onLock: () => void,
  timeoutMinutes: number = 5,
  isEnabled: boolean = true
) {
  const timeoutRef = useRef<number | null>(null);

  const resetTimer = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    if (isEnabled) {
      timeoutRef.current = setTimeout(() => {
        onLock();
      }, timeoutMinutes * 60 * 1000);
    }
  };

  useEffect(() => {
    if (!isEnabled) {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      return;
    }

    // Événements qui réinitialisent le timer
    const events = ["mousedown", "mousemove", "keypress", "scroll", "touchstart"];

    const handleActivity = () => {
      resetTimer();
    };

    // Ajouter les listeners
    events.forEach((event) => {
      document.addEventListener(event, handleActivity);
    });

    // Démarrer le timer
    resetTimer();

    // Cleanup
    return () => {
      events.forEach((event) => {
        document.removeEventListener(event, handleActivity);
      });
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [timeoutMinutes, isEnabled]);

  return { resetTimer };
}
