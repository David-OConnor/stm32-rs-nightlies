#[doc = "Register `HSLVR` reader"]
pub struct R(crate::R<HSLVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSLVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSLVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSLVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSLVR` writer"]
pub struct W(crate::W<HSLVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSLVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HSLVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSLVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSLV0` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_R = crate::BitReader<bool>;
#[doc = "Field `HSLV0` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV1` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV1_R = crate::BitReader<bool>;
#[doc = "Field `HSLV1` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV2` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV2_R = crate::BitReader<bool>;
#[doc = "Field `HSLV2` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV3` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV3_R = crate::BitReader<bool>;
#[doc = "Field `HSLV3` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV4` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV4_R = crate::BitReader<bool>;
#[doc = "Field `HSLV4` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV5` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV5_R = crate::BitReader<bool>;
#[doc = "Field `HSLV5` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV6` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV6_R = crate::BitReader<bool>;
#[doc = "Field `HSLV6` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV7` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV7_R = crate::BitReader<bool>;
#[doc = "Field `HSLV7` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV8` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV8_R = crate::BitReader<bool>;
#[doc = "Field `HSLV8` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV9` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV9_R = crate::BitReader<bool>;
#[doc = "Field `HSLV9` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV10` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV10_R = crate::BitReader<bool>;
#[doc = "Field `HSLV10` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV11` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV11_R = crate::BitReader<bool>;
#[doc = "Field `HSLV11` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV12` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV12_R = crate::BitReader<bool>;
#[doc = "Field `HSLV12` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV13` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV13_R = crate::BitReader<bool>;
#[doc = "Field `HSLV13` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV14` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV14_R = crate::BitReader<bool>;
#[doc = "Field `HSLV14` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
#[doc = "Field `HSLV15` reader - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV15_R = crate::BitReader<bool>;
#[doc = "Field `HSLV15` writer - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type HSLV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSLVR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv0(&self) -> HSLV0_R {
        HSLV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv1(&self) -> HSLV1_R {
        HSLV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv2(&self) -> HSLV2_R {
        HSLV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv3(&self) -> HSLV3_R {
        HSLV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv4(&self) -> HSLV4_R {
        HSLV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv5(&self) -> HSLV5_R {
        HSLV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv6(&self) -> HSLV6_R {
        HSLV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv7(&self) -> HSLV7_R {
        HSLV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv8(&self) -> HSLV8_R {
        HSLV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv9(&self) -> HSLV9_R {
        HSLV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv10(&self) -> HSLV10_R {
        HSLV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv11(&self) -> HSLV11_R {
        HSLV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv12(&self) -> HSLV12_R {
        HSLV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv13(&self) -> HSLV13_R {
        HSLV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv14(&self) -> HSLV14_R {
        HSLV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn hslv15(&self) -> HSLV15_R {
        HSLV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv0(&mut self) -> HSLV0_W<0> {
        HSLV0_W::new(self)
    }
    #[doc = "Bit 1 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv1(&mut self) -> HSLV1_W<1> {
        HSLV1_W::new(self)
    }
    #[doc = "Bit 2 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv2(&mut self) -> HSLV2_W<2> {
        HSLV2_W::new(self)
    }
    #[doc = "Bit 3 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv3(&mut self) -> HSLV3_W<3> {
        HSLV3_W::new(self)
    }
    #[doc = "Bit 4 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv4(&mut self) -> HSLV4_W<4> {
        HSLV4_W::new(self)
    }
    #[doc = "Bit 5 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv5(&mut self) -> HSLV5_W<5> {
        HSLV5_W::new(self)
    }
    #[doc = "Bit 6 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv6(&mut self) -> HSLV6_W<6> {
        HSLV6_W::new(self)
    }
    #[doc = "Bit 7 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv7(&mut self) -> HSLV7_W<7> {
        HSLV7_W::new(self)
    }
    #[doc = "Bit 8 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv8(&mut self) -> HSLV8_W<8> {
        HSLV8_W::new(self)
    }
    #[doc = "Bit 9 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv9(&mut self) -> HSLV9_W<9> {
        HSLV9_W::new(self)
    }
    #[doc = "Bit 10 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv10(&mut self) -> HSLV10_W<10> {
        HSLV10_W::new(self)
    }
    #[doc = "Bit 11 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv11(&mut self) -> HSLV11_W<11> {
        HSLV11_W::new(self)
    }
    #[doc = "Bit 12 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv12(&mut self) -> HSLV12_W<12> {
        HSLV12_W::new(self)
    }
    #[doc = "Bit 13 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv13(&mut self) -> HSLV13_W<13> {
        HSLV13_W::new(self)
    }
    #[doc = "Bit 14 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv14(&mut self) -> HSLV14_W<14> {
        HSLV14_W::new(self)
    }
    #[doc = "Bit 15 - Port x high-speed low-voltage configuration (y = 15 to 0) These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn hslv15(&mut self) -> HSLV15_W<15> {
        HSLV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO high-speed low-voltage register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hslvr](index.html) module"]
pub struct HSLVR_SPEC;
impl crate::RegisterSpec for HSLVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hslvr::R](R) reader structure"]
impl crate::Readable for HSLVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hslvr::W](W) writer structure"]
impl crate::Writable for HSLVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSLVR to value 0"]
impl crate::Resettable for HSLVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
