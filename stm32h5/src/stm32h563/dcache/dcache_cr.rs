#[doc = "Register `DCACHE_CR` reader"]
pub struct R(crate::R<DCACHE_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_CR` writer"]
pub struct W(crate::W<DCACHE_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CR_SPEC>;
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
impl From<crate::W<DCACHE_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `CACHEINV` writer - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
pub type CACHEINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `CACHECMD` reader - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
pub type CACHECMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CACHECMD` writer - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
pub type CACHECMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCACHE_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STARTCMD` writer - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
pub type STARTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `RHITMEN` reader - read-hit monitor enable"]
pub type RHITMEN_R = crate::BitReader<bool>;
#[doc = "Field `RHITMEN` writer - read-hit monitor enable"]
pub type RHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `RMISSMEN` reader - read-miss monitor enable"]
pub type RMISSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RMISSMEN` writer - read-miss monitor enable"]
pub type RMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `RHITMRST` reader - read-hit monitor reset"]
pub type RHITMRST_R = crate::BitReader<bool>;
#[doc = "Field `RHITMRST` writer - read-hit monitor reset"]
pub type RHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `RMISSMRST` reader - read-miss monitor reset"]
pub type RMISSMRST_R = crate::BitReader<bool>;
#[doc = "Field `RMISSMRST` writer - read-miss monitor reset"]
pub type RMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `WHITMEN` reader - write-hit monitor enable"]
pub type WHITMEN_R = crate::BitReader<bool>;
#[doc = "Field `WHITMEN` writer - write-hit monitor enable"]
pub type WHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `WMISSMEN` reader - write-miss monitor enable"]
pub type WMISSMEN_R = crate::BitReader<bool>;
#[doc = "Field `WMISSMEN` writer - write-miss monitor enable"]
pub type WMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `WHITMRST` reader - write-hit monitor reset"]
pub type WHITMRST_R = crate::BitReader<bool>;
#[doc = "Field `WHITMRST` writer - write-hit monitor reset"]
pub type WHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `WMISSMRST` reader - write-miss monitor reset"]
pub type WMISSMRST_R = crate::BitReader<bool>;
#[doc = "Field `WMISSMRST` writer - write-miss monitor reset"]
pub type WMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
#[doc = "Field `HBURST` reader - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
pub type HBURST_R = crate::BitReader<bool>;
#[doc = "Field `HBURST` writer - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - read-hit monitor enable"]
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - read-miss monitor enable"]
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - read-hit monitor reset"]
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - read-miss monitor reset"]
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - write-hit monitor enable"]
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - write-miss monitor enable"]
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - write-hit monitor reset"]
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - write-miss monitor reset"]
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<1> {
        CACHEINV_W::new(self)
    }
    #[doc = "Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cachecmd(&mut self) -> CACHECMD_W<8> {
        CACHECMD_W::new(self)
    }
    #[doc = "Bit 11 - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn startcmd(&mut self) -> STARTCMD_W<11> {
        STARTCMD_W::new(self)
    }
    #[doc = "Bit 16 - read-hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmen(&mut self) -> RHITMEN_W<16> {
        RHITMEN_W::new(self)
    }
    #[doc = "Bit 17 - read-miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<17> {
        RMISSMEN_W::new(self)
    }
    #[doc = "Bit 18 - read-hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<18> {
        RHITMRST_W::new(self)
    }
    #[doc = "Bit 19 - read-miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<19> {
        RMISSMRST_W::new(self)
    }
    #[doc = "Bit 20 - write-hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn whitmen(&mut self) -> WHITMEN_W<20> {
        WHITMEN_W::new(self)
    }
    #[doc = "Bit 21 - write-miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<21> {
        WMISSMEN_W::new(self)
    }
    #[doc = "Bit 22 - write-hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn whitmrst(&mut self) -> WHITMRST_W<22> {
        WHITMRST_W::new(self)
    }
    #[doc = "Bit 23 - write-miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<23> {
        WMISSMRST_W::new(self)
    }
    #[doc = "Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCACHE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_cr](index.html) module"]
pub struct DCACHE_CR_SPEC;
impl crate::RegisterSpec for DCACHE_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_cr::R](R) reader structure"]
impl crate::Readable for DCACHE_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_cr::W](W) writer structure"]
impl crate::Writable for DCACHE_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_CR to value 0"]
impl crate::Resettable for DCACHE_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
