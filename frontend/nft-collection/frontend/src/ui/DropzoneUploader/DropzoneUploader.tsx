import { useDropzone } from 'react-dropzone';
import { MouseEvent, useCallback, useEffect, useState } from 'react';
import { useAtomValue } from 'jotai';
import { cx } from '@/utils';
import picImage from '@/assets/icons/picture.png';
import closeIcon from '@/assets/icons/cross-icon.svg';
import styles from './DropzoneUploader.module.scss';
import { Button } from '../Button';
import { DropzoneUploaderProps } from './DropzoneUploader.interface';
import { IPFS_ATOM } from '@/atoms';

function DropzoneUploader({ content, previewLink, onDropFile, className, multi }: DropzoneUploaderProps) {
  const ipfs = useAtomValue(IPFS_ATOM);
  // const uploadUrl = 'http://127.0.0.1:5001/api/v0/add';
  const uploadUrl = `${ipfs.address}/add`;
  const [preview, setPreview] = useState<string[]>([]);

  useEffect(() => {
    onDropFile(preview);
  }, [preview, onDropFile]);

  const onDrop = (acceptedFiles: File[]) => {
    const formData = new FormData();
    formData.append('file', acceptedFiles[0]);

    fetch(uploadUrl, {
      method: 'POST',
      body: formData,
    })
      .then((res) => res.json())
      .then(({ Hash }) => {
        const link = `${ipfs.gateway}/`;

        if (multi) {
          setPreview((prev) => [...prev, `${link}${Hash}`]);
        } else {
          setPreview([`${link}${Hash}`]);
        }
      });
  };

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    maxFiles: multi ? undefined : 1,
    accept: {
      'image/png': [],
      'image/jpg': [],
      'image/jpeg': [],
    },
    minSize: 760,
  });

  const handleRemovePreview = (e: MouseEvent<HTMLButtonElement>, link?: string) => {
    if (multi) {
      setPreview((prev) => prev.filter((item) => item !== link));
    } else {
      setPreview([]);
    }
  };

  return (
    <div>
      <div {...getRootProps()} className={cx(styles.dropzone, className || '')}>
        {!preview.length || multi ? (
          <>
            <input {...getInputProps()} />
            {isDragActive ? (
              <div className={cx(styles.label)}>
                <p className={cx(styles['label-title'])}>Drop the files here ...</p>
              </div>
            ) : (
              <>
                {!content ? (
                  <div className={cx(styles.label)}>
                    <img src={picImage} alt="upload" />
                    <h5 className={cx(styles['label-title'])}>Upload photo</h5>
                    <p className={cx(styles['label-description'])}>
                      Image not less than 1280x760 in JPG, JPEG or PNG format, up to 1 MB in size
                    </p>
                  </div>
                ) : (
                  <>{content}</>
                )}
              </>
            )}
          </>
        ) : null}
        {!!preview.length && !multi && (
          <div className={cx(styles['uploaded-pic-wrapper'])}>
            <img src={preview[0]} alt="preview" className={cx(styles['uploaded-pic'])} />
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

      {!!preview.length && multi && (
        <div className={cx(styles['multi-previews-wrapper'])}>
          {preview.map((link) => (
            <div key={link} className={cx(styles['uploaded-pic-wrapper-multi'])}>
              <img src={link} alt="preview" className={cx(styles['uploaded-pic-multi'])} />
              <Button
                icon={closeIcon}
                variant="text"
                className={cx(styles['close-icon-multi'])}
                label=""
                onClick={(e) => handleRemovePreview(e, link)}
              />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export { DropzoneUploader };
