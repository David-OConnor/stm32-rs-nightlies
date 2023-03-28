#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAR` reader - Own Address"]
pub type OAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OAR` writer - Own Address"]
pub type OAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LSTN` reader - Listen mode"]
pub type LSTN_R = crate::BitReader<bool>;
#[doc = "Field `LSTN` writer - Listen mode"]
pub type LSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub type RXTOL_R = crate::BitReader<bool>;
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub type RXTOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BRESTP` reader - Rx-stop on bit rising error"]
pub type BRESTP_R = crate::BitReader<bool>;
#[doc = "Field `BRESTP` writer - Rx-stop on bit rising error"]
pub type BRESTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BREGEN` reader - Generate error-bit on bit rising error"]
pub type BREGEN_R = crate::BitReader<bool>;
#[doc = "Field `BREGEN` writer - Generate error-bit on bit rising error"]
pub type BREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_R = crate::BitReader<bool>;
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Own Address"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx-stop on bit rising error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generate error-bit on bit rising error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Own Address"]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<0> {
        OAR_W::new(self)
    }
    #[doc = "Bit 4 - Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<4> {
        LSTN_W::new(self)
    }
    #[doc = "Bits 5:7 - Signal Free Time"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<5> {
        SFT_W::new(self)
    }
    #[doc = "Bit 8 - Rx-Tolerance"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<8> {
        RXTOL_W::new(self)
    }
    #[doc = "Bit 9 - Rx-stop on bit rising error"]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<9> {
        BRESTP_W::new(self)
    }
    #[doc = "Bit 10 - Generate error-bit on bit rising error"]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<10> {
        BREGEN_W::new(self)
    }
    #[doc = "Bit 11 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<11> {
        LBPEGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
