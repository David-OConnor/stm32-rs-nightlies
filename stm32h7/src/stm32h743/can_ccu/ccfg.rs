#[doc = "Register `CCFG` reader"]
pub struct R(crate::R<CCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG` writer"]
pub struct W(crate::W<CCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SPEC>;
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
impl From<crate::W<CCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TQBT` reader - Time Quanta per Bit Time"]
pub type TQBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TQBT` writer - Time Quanta per Bit Time"]
pub type TQBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `BCC` reader - Bypass Clock Calibration"]
pub type BCC_R = crate::BitReader<bool>;
#[doc = "Field `BCC` writer - Bypass Clock Calibration"]
pub type BCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SPEC, bool, O>;
#[doc = "Field `CFL` reader - Calibration Field Length"]
pub type CFL_R = crate::BitReader<bool>;
#[doc = "Field `CFL` writer - Calibration Field Length"]
pub type CFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SPEC, bool, O>;
#[doc = "Field `OCPM` reader - Oscillator Clock Periods Minimum"]
pub type OCPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCPM` writer - Oscillator Clock Periods Minimum"]
pub type OCPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `CDIV` reader - Clock Divider"]
pub type CDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CDIV` writer - Clock Divider"]
pub type CDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    #[must_use]
    pub fn tqbt(&mut self) -> TQBT_W<0> {
        TQBT_W::new(self)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn bcc(&mut self) -> BCC_W<6> {
        BCC_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    #[must_use]
    pub fn cfl(&mut self) -> CFL_W<7> {
        CFL_W::new(self)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OCPM_W<8> {
        OCPM_W::new(self)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CDIV_W<16> {
        CDIV_W::new(self)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<31> {
        SWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg](index.html) module"]
pub struct CCFG_SPEC;
impl crate::RegisterSpec for CCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg::R](R) reader structure"]
impl crate::Readable for CCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg::W](W) writer structure"]
impl crate::Writable for CCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG to value 0"]
impl crate::Resettable for CCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
