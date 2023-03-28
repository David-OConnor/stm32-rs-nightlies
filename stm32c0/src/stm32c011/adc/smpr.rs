#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMPSEL0` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL0_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL0` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL1` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL1_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL1` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL2` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL2_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL2` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL3` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL3_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL3` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL4` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL4_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL4` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL5` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL5_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL5` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL6` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL6_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL6` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL7` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL7_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL7` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL8` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL8_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL8` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL9` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL9_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL9` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL10` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL10_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL10` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL11` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL11_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL11` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL12` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL12_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL12` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL13` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL13_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL13` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL14` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL14_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL14` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL15` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL15_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL15` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL16` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL16_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL16` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL17` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL17_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL17` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL18` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL18_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL18` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL19` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL19_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL19` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL20` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL20_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL20` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL21` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL21_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL21` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
#[doc = "Field `SMPSEL22` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL22_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL22` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
pub type SMPSEL22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL19_R {
        SMPSEL19_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel20(&self) -> SMPSEL20_R {
        SMPSEL20_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel21(&self) -> SMPSEL21_R {
        SMPSEL21_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    pub fn smpsel22(&self) -> SMPSEL22_R {
        SMPSEL22_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<0> {
        SMP1_W::new(self)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<4> {
        SMP2_W::new(self)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<8> {
        SMPSEL0_W::new(self)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<9> {
        SMPSEL1_W::new(self)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<10> {
        SMPSEL2_W::new(self)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<11> {
        SMPSEL3_W::new(self)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<12> {
        SMPSEL4_W::new(self)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<13> {
        SMPSEL5_W::new(self)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<14> {
        SMPSEL6_W::new(self)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<15> {
        SMPSEL7_W::new(self)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<16> {
        SMPSEL8_W::new(self)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<17> {
        SMPSEL9_W::new(self)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<18> {
        SMPSEL10_W::new(self)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<19> {
        SMPSEL11_W::new(self)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<20> {
        SMPSEL12_W::new(self)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<21> {
        SMPSEL13_W::new(self)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<22> {
        SMPSEL14_W::new(self)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<23> {
        SMPSEL15_W::new(self)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<24> {
        SMPSEL16_W::new(self)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<25> {
        SMPSEL17_W::new(self)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<26> {
        SMPSEL18_W::new(self)
    }
    #[doc = "Bit 27 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel19(&mut self) -> SMPSEL19_W<27> {
        SMPSEL19_W::new(self)
    }
    #[doc = "Bit 28 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel20(&mut self) -> SMPSEL20_W<28> {
        SMPSEL20_W::new(self)
    }
    #[doc = "Bit 29 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel21(&mut self) -> SMPSEL21_W<29> {
        SMPSEL21_W::new(self)
    }
    #[doc = "Bit 30 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Refer to for the maximum number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel22(&mut self) -> SMPSEL22_W<30> {
        SMPSEL22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}