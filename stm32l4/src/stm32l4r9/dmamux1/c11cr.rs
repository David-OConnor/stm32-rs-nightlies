#[doc = "Register `C11CR` reader"]
pub struct R(crate::R<C11CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C11CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C11CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C11CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C11CR` writer"]
pub struct W(crate::W<C11CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C11CR_SPEC>;
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
impl From<crate::W<C11CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C11CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAREQ_ID` reader - DMA request identification"]
pub type DMAREQ_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMAREQ_ID` writer - DMA request identification"]
pub type DMAREQ_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C11CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SOIE_R = crate::BitReader<bool>;
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C11CR_SPEC, bool, O>;
#[doc = "Field `EGE` reader - Event generation enable"]
pub type EGE_R = crate::BitReader<bool>;
#[doc = "Field `EGE` writer - Event generation enable"]
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C11CR_SPEC, bool, O>;
#[doc = "Field `SE` reader - Synchronization enable"]
pub type SE_R = crate::BitReader<bool>;
#[doc = "Field `SE` writer - Synchronization enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C11CR_SPEC, bool, O>;
#[doc = "Field `SPOL` reader - Synchronization polarity"]
pub type SPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPOL` writer - Synchronization polarity"]
pub type SPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C11CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NBREQ` reader - Number of DMA requests minus 1 to forward"]
pub type NBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBREQ` writer - Number of DMA requests minus 1 to forward"]
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C11CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SYNC_ID` reader - Synchronization identification"]
pub type SYNC_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC_ID` writer - Synchronization identification"]
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C11CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:6 - DMA request identification"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward"]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request identification"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward"]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    #[doc = "Bits 24:28 - Synchronization identification"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel 11 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11cr](index.html) module"]
pub struct C11CR_SPEC;
impl crate::RegisterSpec for C11CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c11cr::R](R) reader structure"]
impl crate::Readable for C11CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c11cr::W](W) writer structure"]
impl crate::Writable for C11CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C11CR to value 0"]
impl crate::Resettable for C11CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
