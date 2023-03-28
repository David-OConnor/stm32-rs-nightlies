#[doc = "Register `SAI_AIM` reader"]
pub struct R(crate::R<SAI_AIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_AIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_AIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_AIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_AIM` writer"]
pub struct W(crate::W<SAI_AIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_AIM_SPEC>;
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
impl From<crate::W<SAI_AIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_AIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDRIE` reader - OVRUDRIE"]
pub type OVRUDRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRUDRIE` writer - OVRUDRIE"]
pub type OVRUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `MUTEDETIE` reader - MUTEDETIE"]
pub type MUTEDETIE_R = crate::BitReader<bool>;
#[doc = "Field `MUTEDETIE` writer - MUTEDETIE"]
pub type MUTEDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `WCKCFGIE` reader - WCKCFGIE"]
pub type WCKCFGIE_R = crate::BitReader<bool>;
#[doc = "Field `WCKCFGIE` writer - WCKCFGIE"]
pub type WCKCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `FREQIE` reader - FREQIE"]
pub type FREQIE_R = crate::BitReader<bool>;
#[doc = "Field `FREQIE` writer - FREQIE"]
pub type FREQIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `CNRDYIE` reader - CNRDYIE"]
pub type CNRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `CNRDYIE` writer - CNRDYIE"]
pub type CNRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `AFSDETIE` reader - AFSDETIE"]
pub type AFSDETIE_R = crate::BitReader<bool>;
#[doc = "Field `AFSDETIE` writer - AFSDETIE"]
pub type AFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
#[doc = "Field `LFSDETIE` reader - LFSDETIE"]
pub type LFSDETIE_R = crate::BitReader<bool>;
#[doc = "Field `LFSDETIE` writer - LFSDETIE"]
pub type LFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<0> {
        OVRUDRIE_W::new(self)
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<1> {
        MUTEDETIE_W::new(self)
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<2> {
        WCKCFGIE_W::new(self)
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<3> {
        FREQIE_W::new(self)
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<4> {
        CNRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<5> {
        AFSDETIE_W::new(self)
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<6> {
        LFSDETIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aim](index.html) module"]
pub struct SAI_AIM_SPEC;
impl crate::RegisterSpec for SAI_AIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_aim::R](R) reader structure"]
impl crate::Readable for SAI_AIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_aim::W](W) writer structure"]
impl crate::Writable for SAI_AIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_AIM to value 0"]
impl crate::Resettable for SAI_AIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
