import { useDropzone } from 'react-dropzone';
import { MouseEvent, useCallback, useEffect, useState } from 'react';
import { cx } from '@/utils';
import picImage from '@/assets/icons/picture.png';
import closeIcon from '@/assets/icons/cross-icon.svg';
import styles from './DropzoneUploader.module.scss';
import { Button } from '../Button';
import { DropzoneUploaderProps } from './DropzoneUploader.interface';
import { ADDRESS } from '@/consts';

function DropzoneUploader({ text, previewLink, onDropFile }: DropzoneUploaderProps) {
  const uploadUrl = ADDRESS.IPFS_NODE;

  const [preview, setPreview] = useState(previewLink || '');

  useEffect(() => {
    if (preview) {
      onDropFile(preview);
    }
  }, [preview, onDropFile]);

  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      const formData = new FormData();
      formData.append('file', acceptedFiles[0]);

      fetch(uploadUrl, {
        method: 'POST',
        body: formData,
      })
        .then((res) => res.json())
        .then(({ Hash }) => setPreview(`${ADDRESS.IPFS_GATEWAY}/${Hash}`));
    },
    [uploadUrl],
  );

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    maxFiles: 1,
    accept: {
      'image/png': [],
      'image/jpg': [],
      'image/jpeg': [],
    },
    minSize: 760,
  });

  const handleRemovePreview = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    setPreview('');
  };

  return (
    <div {...getRootProps()} className={cx(styles.dropzone)}>
      {!preview ? (
        <>
          <input {...getInputProps()} />
          {isDragActive ? (
            <div className={cx(styles.label)}>
              <p className={cx(styles['label-title'])}>Drop the files here ...</p>
            </div>
          ) : (
            <div className={cx(styles.label)}>
              <img src={picImage} alt="upload" />
              <h5 className={cx(styles['label-title'])}>Upload photo</h5>
              <p className={cx(styles['label-description'])}>
                {text !== undefined || 'Image not less than 1280x760 in JPG, JPEG or PNG format, up to 1 MB in size'}
              </p>
            </div>
          )}
        </>
      ) : null}
      {preview && (
        <div className={cx(styles['uploaded-pic-wrapper'])}>
          <img src={preview} alt="preview" className={cx(styles['uploaded-pic'])} />
          <Button
            icon={closeIcon}
            variant="text"
            className={cx(styles['close-icon'])}
            label=""
            onClick={handleRemovePreview}
          />
        </div>
      )}
    </div>
  );
}

export { DropzoneUploader };
