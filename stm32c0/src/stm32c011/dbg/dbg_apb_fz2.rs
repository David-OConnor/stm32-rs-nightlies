#[doc = "Register `DBG_APB_FZ2` reader"]
pub struct R(crate::R<DBG_APB_FZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_APB_FZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_APB_FZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_APB_FZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_APB_FZ2` writer"]
pub struct W(crate::W<DBG_APB_FZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_APB_FZ2_SPEC>;
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
impl From<crate::W<DBG_APB_FZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_APB_FZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_APB_FZ2_SPEC, bool, O>;
#[doc = "Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_APB_FZ2_SPEC, bool, O>;
#[doc = "Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_APB_FZ2_SPEC, bool, O>;
#[doc = "Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_APB_FZ2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<11> {
        DBG_TIM1_STOP_W::new(self)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<15> {
        DBG_TIM14_STOP_W::new(self)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<17> {
        DBG_TIM16_STOP_W::new(self)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<18> {
        DBG_TIM17_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG APB freeze register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_apb_fz2](index.html) module"]
pub struct DBG_APB_FZ2_SPEC;
impl crate::RegisterSpec for DBG_APB_FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_apb_fz2::R](R) reader structure"]
impl crate::Readable for DBG_APB_FZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_apb_fz2::W](W) writer structure"]
impl crate::Writable for DBG_APB_FZ2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ2 to value 0"]
impl crate::Resettable for DBG_APB_FZ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
