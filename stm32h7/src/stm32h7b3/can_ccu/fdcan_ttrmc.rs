#[doc = "Register `FDCAN_TTRMC` reader"]
pub struct R(crate::R<FDCAN_TTRMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTRMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTRMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTRMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTRMC` writer"]
pub struct W(crate::W<FDCAN_TTRMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTRMC_SPEC>;
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
impl From<crate::W<FDCAN_TTRMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTRMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RID` reader - Reference Identifier."]
pub type RID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RID` writer - Reference Identifier."]
pub type RID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTRMC_SPEC, u32, u32, 29, O>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XTD_R = crate::BitReader<bool>;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTRMC_SPEC, bool, O>;
#[doc = "Field `RMPS` reader - Reference Message Payload Select"]
pub type RMPS_R = crate::BitReader<bool>;
#[doc = "Field `RMPS` writer - Reference Message Payload Select"]
pub type RMPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTRMC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    #[must_use]
    pub fn rid(&mut self) -> RID_W<0> {
        RID_W::new(self)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn xtd(&mut self) -> XTD_W<30> {
        XTD_W::new(self)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    #[must_use]
    pub fn rmps(&mut self) -> RMPS_W<31> {
        RMPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Reference Message Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttrmc](index.html) module"]
pub struct FDCAN_TTRMC_SPEC;
impl crate::RegisterSpec for FDCAN_TTRMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttrmc::R](R) reader structure"]
impl crate::Readable for FDCAN_TTRMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttrmc::W](W) writer structure"]
impl crate::Writable for FDCAN_TTRMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTRMC to value 0"]
impl crate::Resettable for FDCAN_TTRMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
