/**
 * 存储项的包装结构，方便以后扩展（如添加过期时间）
 */
interface StorageData<T> {
  value: T;
  version: string;
}

class StorageUtil {
  private static readonly VERSION = '1.0.0';

  /**
   * 设置存储项
   * @param key 键名
   * @param value 值（支持各种类型）
   */
  static set<T>(key: string, value: T): void {
    try {
      const data: StorageData<T> = {
        value,
        version: this.VERSION
      };
      localStorage.setItem(key, JSON.stringify(data));
    } catch (error) {
      console.error(`LocalStorage Set Error: ${error}`);
    }
  }

  /**
   * 获取存储项
   * @param key 键名
   * @returns 泛型 T 或 null
   */
  static get<T>(key: string): T | null {
    const raw = localStorage.getItem(key);
    if (!raw) return null;

    try {
      const data: StorageData<T> = JSON.parse(raw);
      return data.value;
    } catch (error) {
      console.error(`LocalStorage Get Error: ${error}`);
      return null;
    }
  }

  /**
   * 移除指定项
   */
  static remove(key: string): void {
    localStorage.removeItem(key);
  }

  /**
   * 清空所有存储
   */
  static clear(): void {
    localStorage.clear();
  }
}

export default StorageUtil;