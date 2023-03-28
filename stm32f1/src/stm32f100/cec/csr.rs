#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSOM` reader - Tx start of message"]
pub type TSOM_R = crate::BitReader<bool>;
#[doc = "Field `TSOM` writer - Tx start of message"]
pub type TSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `TEOM` reader - Tx end of message"]
pub type TEOM_R = crate::BitReader<bool>;
#[doc = "Field `TEOM` writer - Tx end of message"]
pub type TEOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `TERR` reader - Tx error"]
pub type TERR_R = crate::BitReader<bool>;
#[doc = "Field `TERR` writer - Tx error"]
pub type TERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `TBTRF` reader - Tx byte transfer request or block transfer finished"]
pub type TBTRF_R = crate::BitReader<bool>;
#[doc = "Field `TBTRF` writer - Tx byte transfer request or block transfer finished"]
pub type TBTRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `RSOM` reader - Rx start of message"]
pub type RSOM_R = crate::BitReader<bool>;
#[doc = "Field `RSOM` writer - Rx start of message"]
pub type RSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `REOM` reader - Rx end of message"]
pub type REOM_R = crate::BitReader<bool>;
#[doc = "Field `REOM` writer - Rx end of message"]
pub type REOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `RERR` reader - Rx error"]
pub type RERR_R = crate::BitReader<bool>;
#[doc = "Field `RERR` writer - Rx error"]
pub type RERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `RBTF` reader - Rx byte/block transfer finished"]
pub type RBTF_R = crate::BitReader<bool>;
#[doc = "Field `RBTF` writer - Rx byte/block transfer finished"]
pub type RBTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&self) -> TSOM_R {
        TSOM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&self) -> TEOM_R {
        TEOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&self) -> TBTRF_R {
        TBTRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&self) -> RSOM_R {
        RSOM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&self) -> REOM_R {
        REOM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&self) -> RBTF_R {
        RBTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    #[must_use]
    pub fn tsom(&mut self) -> TSOM_W<0> {
        TSOM_W::new(self)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    #[must_use]
    pub fn teom(&mut self) -> TEOM_W<1> {
        TEOM_W::new(self)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<2> {
        TERR_W::new(self)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tbtrf(&mut self) -> TBTRF_W<3> {
        TBTRF_W::new(self)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    #[must_use]
    pub fn rsom(&mut self) -> RSOM_W<4> {
        RSOM_W::new(self)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    #[must_use]
    pub fn reom(&mut self) -> REOM_W<5> {
        REOM_W::new(self)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RERR_W<6> {
        RERR_W::new(self)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn rbtf(&mut self) -> RBTF_W<7> {
        RBTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
