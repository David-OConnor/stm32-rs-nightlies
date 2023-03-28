#[doc = "Register `TSIZ` reader"]
pub struct R(crate::R<TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSIZ` writer"]
pub struct W(crate::W<TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSIZ_SPEC>;
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
impl From<crate::W<TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSIZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSIZ_SPEC, bool, O>;
#[doc = "Field `STUPCNT` reader - SETUP packet count"]
pub type STUPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STUPCNT` writer - SETUP packet count"]
pub type STUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSIZ_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn stupcnt(&mut self) -> STUPCNT_W<29> {
        STUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsiz](index.html) module"]
pub struct TSIZ_SPEC;
impl crate::RegisterSpec for TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsiz::R](R) reader structure"]
impl crate::Readable for TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsiz::W](W) writer structure"]
impl crate::Writable for TSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSIZ to value 0"]
impl crate::Resettable for TSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
