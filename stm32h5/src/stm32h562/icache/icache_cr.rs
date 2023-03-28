#[doc = "Register `ICACHE_CR` reader"]
pub struct R(crate::R<ICACHE_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CR` writer"]
pub struct W(crate::W<ICACHE_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CR_SPEC>;
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
impl From<crate::W<ICACHE_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `CACHEINV` writer - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
pub type CACHEINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `WAYSEL` reader - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_R = crate::BitReader<bool>;
#[doc = "Field `WAYSEL` writer - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `HITMEN` reader - hit monitor enable"]
pub type HITMEN_R = crate::BitReader<bool>;
#[doc = "Field `HITMEN` writer - hit monitor enable"]
pub type HITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `MISSMEN` reader - miss monitor enable"]
pub type MISSMEN_R = crate::BitReader<bool>;
#[doc = "Field `MISSMEN` writer - miss monitor enable"]
pub type MISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `HITMRST` reader - hit monitor reset"]
pub type HITMRST_R = crate::BitReader<bool>;
#[doc = "Field `HITMRST` writer - hit monitor reset"]
pub type HITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
#[doc = "Field `MISSMRST` reader - miss monitor reset"]
pub type MISSMRST_R = crate::BitReader<bool>;
#[doc = "Field `MISSMRST` writer - miss monitor reset"]
pub type MISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<1> {
        CACHEINV_W::new(self)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn waysel(&mut self) -> WAYSEL_W<2> {
        WAYSEL_W::new(self)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn hitmen(&mut self) -> HITMEN_W<16> {
        HITMEN_W::new(self)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn missmen(&mut self) -> MISSMEN_W<17> {
        MISSMEN_W::new(self)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn hitmrst(&mut self) -> HITMRST_W<18> {
        HITMRST_W::new(self)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn missmrst(&mut self) -> MISSMRST_W<19> {
        MISSMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_cr](index.html) module"]
pub struct ICACHE_CR_SPEC;
impl crate::RegisterSpec for ICACHE_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_cr::R](R) reader structure"]
impl crate::Readable for ICACHE_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_cr::W](W) writer structure"]
impl crate::Writable for ICACHE_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_CR to value 0x04"]
impl crate::Resettable for ICACHE_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
