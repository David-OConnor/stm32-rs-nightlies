#[doc = "Register `CEC_CFGR` reader"]
pub struct R(crate::R<CEC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CFGR` writer"]
pub struct W(crate::W<CEC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CFGR_SPEC>;
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
impl From<crate::W<CEC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFT` reader - SFT"]
pub type SFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFT` writer - SFT"]
pub type SFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXTOL` reader - RXTOL"]
pub type RXTOL_R = crate::BitReader<bool>;
#[doc = "Field `RXTOL` writer - RXTOL"]
pub type RXTOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRESTP` reader - BRESTP"]
pub type BRESTP_R = crate::BitReader<bool>;
#[doc = "Field `BRESTP` writer - BRESTP"]
pub type BRESTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BREGEN` reader - BREGEN"]
pub type BREGEN_R = crate::BitReader<bool>;
#[doc = "Field `BREGEN` writer - BREGEN"]
pub type BREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `LBPEGEN` reader - LBPEGEN"]
pub type LBPEGEN_R = crate::BitReader<bool>;
#[doc = "Field `LBPEGEN` writer - LBPEGEN"]
pub type LBPEGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRDNOGEN` reader - BRDNOGEN"]
pub type BRDNOGEN_R = crate::BitReader<bool>;
#[doc = "Field `BRDNOGEN` writer - BRDNOGEN"]
pub type BRDNOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `SFTOP` reader - SFTOP"]
pub type SFTOP_R = crate::BitReader<bool>;
#[doc = "Field `SFTOP` writer - SFTOP"]
pub type SFTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `OAR` reader - OAR"]
pub type OAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OAR` writer - OAR"]
pub type OAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u16, u16, 15, O>;
#[doc = "Field `LSTN` reader - LSTN"]
pub type LSTN_R = crate::BitReader<bool>;
#[doc = "Field `LSTN` writer - LSTN"]
pub type LSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - SFT"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RXTOL"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRESTP"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BREGEN"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LBPEGEN"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRDNOGEN"]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SFTOP"]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - OAR"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - LSTN"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SFT"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<0> {
        SFT_W::new(self)
    }
    #[doc = "Bit 3 - RXTOL"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<3> {
        RXTOL_W::new(self)
    }
    #[doc = "Bit 4 - BRESTP"]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<4> {
        BRESTP_W::new(self)
    }
    #[doc = "Bit 5 - BREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<5> {
        BREGEN_W::new(self)
    }
    #[doc = "Bit 6 - LBPEGEN"]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<6> {
        LBPEGEN_W::new(self)
    }
    #[doc = "Bit 7 - BRDNOGEN"]
    #[inline(always)]
    #[must_use]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<7> {
        BRDNOGEN_W::new(self)
    }
    #[doc = "Bit 8 - SFTOP"]
    #[inline(always)]
    #[must_use]
    pub fn sftop(&mut self) -> SFTOP_W<8> {
        SFTOP_W::new(self)
    }
    #[doc = "Bits 16:30 - OAR"]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<16> {
        OAR_W::new(self)
    }
    #[doc = "Bit 31 - LSTN"]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<31> {
        LSTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cfgr](index.html) module"]
pub struct CEC_CFGR_SPEC;
impl crate::RegisterSpec for CEC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cfgr::R](R) reader structure"]
impl crate::Readable for CEC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cfgr::W](W) writer structure"]
impl crate::Writable for CEC_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEC_CFGR to value 0"]
impl crate::Resettable for CEC_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
