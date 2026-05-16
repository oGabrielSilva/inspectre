import { useI18n } from 'vue-i18n';

type FeedbackInput = {
  title?: string;
  description?: string;
  code?: string;
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

  return {
    success(input: FeedbackInput) {
      toast.add({ ...resolve(input), color: 'success' });
    },
    error(input: FeedbackInput) {
      toast.add({ ...resolve(input), color: 'error' });
    },
    warning(input: FeedbackInput) {
      toast.add({ ...resolve(input), color: 'warning' });
    },
    info(input: FeedbackInput) {
      toast.add({ ...resolve(input), color: 'info' });
    },
  };
}
