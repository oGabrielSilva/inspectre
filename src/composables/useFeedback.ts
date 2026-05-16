import { useI18n } from 'vue-i18n';

type FeedbackAction = {
  icon?: string;
  label: string;
  onClick: (e?: Event) => void;
};

type FeedbackInput = {
  title?: string;
  description?: string;
  code?: string;
  actions?: FeedbackAction[];
};

type ResolvedFeedback = {
  title: string;
  description?: string;
};

export function useFeedback() {
  const toast = useToast();
  const { te, t } = useI18n();

  function resolve(input: FeedbackInput): ResolvedFeedback {
    if (input.code) {
      const titleKey = `errors.${input.code}.title`;
      const descKey = `errors.${input.code}.description`;
      return {
        title: te(titleKey) ? t(titleKey) : (input.title ?? input.code),
        description: te(descKey) ? t(descKey) : input.description,
      };
    }
    return {
      title: input.title ?? '',
      description: input.description,
    };
  }

  function payload(input: FeedbackInput, color: 'success' | 'error' | 'warning' | 'info') {
    const base = { ...resolve(input), color };
    return input.actions ? { ...base, actions: input.actions } : base;
  }

  return {
    success(input: FeedbackInput) {
      toast.add(payload(input, 'success'));
    },
    error(input: FeedbackInput) {
      toast.add(payload(input, 'error'));
    },
    warning(input: FeedbackInput) {
      toast.add(payload(input, 'warning'));
    },
    info(input: FeedbackInput) {
      toast.add(payload(input, 'info'));
    },
  };
}
