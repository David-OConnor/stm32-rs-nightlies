#[doc = "Register `DCMI_CR` reader"]
pub struct R(crate::R<DCMI_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_CR` writer"]
pub struct W(crate::W<DCMI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_CR_SPEC>;
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
impl From<crate::W<DCMI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE` reader - CAPTURE"]
pub type CAPTURE_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE` writer - CAPTURE"]
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `CM` reader - CM"]
pub type CM_R = crate::BitReader<bool>;
#[doc = "Field `CM` writer - CM"]
pub type CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `CROP` reader - CROP"]
pub type CROP_R = crate::BitReader<bool>;
#[doc = "Field `CROP` writer - CROP"]
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `ESS` reader - ESS"]
pub type ESS_R = crate::BitReader<bool>;
#[doc = "Field `ESS` writer - ESS"]
pub type ESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `PCKPOL` reader - PCKPOL"]
pub type PCKPOL_R = crate::BitReader<bool>;
#[doc = "Field `PCKPOL` writer - PCKPOL"]
pub type PCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `HSPOL` reader - HSPOL"]
pub type HSPOL_R = crate::BitReader<bool>;
#[doc = "Field `HSPOL` writer - HSPOL"]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `VSPOL` reader - VSPOL"]
pub type VSPOL_R = crate::BitReader<bool>;
#[doc = "Field `VSPOL` writer - VSPOL"]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `FCRC` reader - FCRC"]
pub type FCRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCRC` writer - FCRC"]
pub type FCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDM` reader - EDM"]
pub type EDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDM` writer - EDM"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `BSM` reader - BSM"]
pub type BSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSM` writer - BSM"]
pub type BSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OEBS` reader - OEBS"]
pub type OEBS_R = crate::BitReader<bool>;
#[doc = "Field `OEBS` writer - OEBS"]
pub type OEBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `LSM` reader - LSM"]
pub type LSM_R = crate::BitReader<bool>;
#[doc = "Field `LSM` writer - LSM"]
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `OELS` reader - OELS"]
pub type OELS_R = crate::BitReader<bool>;
#[doc = "Field `OELS` writer - OELS"]
pub type OELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CAPTURE"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CM"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CROP"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ESS"]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCKPOL"]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSPOL"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - FCRC"]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - EDM"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - BSM"]
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - OEBS"]
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LSM"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OELS"]
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAPTURE"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 1 - CM"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<1> {
        CM_W::new(self)
    }
    #[doc = "Bit 2 - CROP"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<2> {
        CROP_W::new(self)
    }
    #[doc = "Bit 3 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<3> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 4 - ESS"]
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<4> {
        ESS_W::new(self)
    }
    #[doc = "Bit 5 - PCKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<5> {
        PCKPOL_W::new(self)
    }
    #[doc = "Bit 6 - HSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<6> {
        HSPOL_W::new(self)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    #[doc = "Bits 8:9 - FCRC"]
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<8> {
        FCRC_W::new(self)
    }
    #[doc = "Bits 10:11 - EDM"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bit 14 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 16:17 - BSM"]
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<16> {
        BSM_W::new(self)
    }
    #[doc = "Bit 18 - OEBS"]
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<18> {
        OEBS_W::new(self)
    }
    #[doc = "Bit 19 - LSM"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<19> {
        LSM_W::new(self)
    }
    #[doc = "Bit 20 - OELS"]
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<20> {
        OELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cr](index.html) module"]
pub struct DCMI_CR_SPEC;
impl crate::RegisterSpec for DCMI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_cr::R](R) reader structure"]
impl crate::Readable for DCMI_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_cr::W](W) writer structure"]
impl crate::Writable for DCMI_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_CR to value 0"]
impl crate::Resettable for DCMI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
