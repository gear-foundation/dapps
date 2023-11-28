export interface DropzoneUploaderProps {
  onDropFile: (prev: string[]) => void;
  previewLink?: string;
  content?: JSX.Element;
  className?: string;
  multi?: boolean;
}
