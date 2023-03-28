#[doc = "Register `EP6R` reader"]
pub struct R(crate::R<EP6R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP6R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP6R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP6R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP6R` writer"]
pub struct W(crate::W<EP6R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP6R_SPEC>;
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
impl From<crate::W<EP6R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP6R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EA` reader - EA"]
pub type EA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EA` writer - EA"]
pub type EA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP6R_SPEC, u8, u8, 4, O>;
#[doc = "Field `STAT_TX` reader - STAT_TX"]
pub type STAT_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT_TX` writer - STAT_TX"]
pub type STAT_TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP6R_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTOG_TX` reader - DTOG_TX"]
pub type DTOG_TX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_TX` writer - DTOG_TX"]
pub type DTOG_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
#[doc = "Field `CTR_TX` reader - CTR_TX"]
pub type CTR_TX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_TX` writer - CTR_TX"]
pub type CTR_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
#[doc = "Field `EP_KIND` reader - EP_KIND"]
pub type EP_KIND_R = crate::BitReader<bool>;
#[doc = "Field `EP_KIND` writer - EP_KIND"]
pub type EP_KIND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
#[doc = "Field `EP_TYPE` reader - EP_TYPE"]
pub type EP_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP_TYPE` writer - EP_TYPE"]
pub type EP_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP6R_SPEC, u8, u8, 2, O>;
#[doc = "Field `SETUP` reader - SETUP"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP"]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
#[doc = "Field `STAT_RX` reader - STAT_RX"]
pub type STAT_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT_RX` writer - STAT_RX"]
pub type STAT_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP6R_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTOG_RX` reader - DTOG_RX"]
pub type DTOG_RX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_RX` writer - DTOG_RX"]
pub type DTOG_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
#[doc = "Field `CTR_RX` reader - CTR_RX"]
pub type CTR_RX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_RX` writer - CTR_RX"]
pub type CTR_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP6R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<0> {
        EA_W::new(self)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<4> {
        STAT_TX_W::new(self)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<6> {
        DTOG_TX_W::new(self)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<7> {
        CTR_TX_W::new(self)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<8> {
        EP_KIND_W::new(self)
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<9> {
        EP_TYPE_W::new(self)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<11> {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<12> {
        STAT_RX_W::new(self)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<14> {
        DTOG_RX_W::new(self)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<15> {
        CTR_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6r](index.html) module"]
pub struct EP6R_SPEC;
impl crate::RegisterSpec for EP6R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep6r::R](R) reader structure"]
impl crate::Readable for EP6R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep6r::W](W) writer structure"]
impl crate::Writable for EP6R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP6R to value 0"]
impl crate::Resettable for EP6R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
