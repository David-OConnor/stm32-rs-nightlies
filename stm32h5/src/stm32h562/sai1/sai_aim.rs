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
#[doc = "Field `OVRUDRIE` reader - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
pub type OVRUDRIE_R = crate::BitReader<OVRUDRIE_A>;
#[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<OVRUDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRUDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRUDRIE_A {
        match self.bits {
            false => OVRUDRIE_A::Disabled,
            true => OVRUDRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRUDRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRUDRIE_A::Enabled
    }
}
#[doc = "Field `OVRUDRIE` writer - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
pub type OVRUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, OVRUDRIE_A, O>;
impl<'a, const O: u8> OVRUDRIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::Enabled)
    }
}
#[doc = "Field `MUTEDETIE` reader - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
pub type MUTEDETIE_R = crate::BitReader<MUTEDETIE_A>;
#[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<MUTEDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTEDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEDETIE_A {
        match self.bits {
            false => MUTEDETIE_A::Disabled,
            true => MUTEDETIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTEDETIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTEDETIE_A::Enabled
    }
}
#[doc = "Field `MUTEDETIE` writer - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
pub type MUTEDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, MUTEDETIE_A, O>;
impl<'a, const O: u8> MUTEDETIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::Enabled)
    }
}
#[doc = "Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
pub type WCKCFGIE_R = crate::BitReader<WCKCFGIE_A>;
#[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<WCKCFGIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WCKCFGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCKCFGIE_A {
        match self.bits {
            false => WCKCFGIE_A::Disabled,
            true => WCKCFGIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WCKCFGIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WCKCFGIE_A::Enabled
    }
}
#[doc = "Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
pub type WCKCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, WCKCFGIE_A, O>;
impl<'a, const O: u8> WCKCFGIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::Enabled)
    }
}
#[doc = "Field `FREQIE` reader - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
pub type FREQIE_R = crate::BitReader<FREQIE_A>;
#[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<FREQIE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQIE_A {
        match self.bits {
            false => FREQIE_A::Disabled,
            true => FREQIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FREQIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FREQIE_A::Enabled
    }
}
#[doc = "Field `FREQIE` writer - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
pub type FREQIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, FREQIE_A, O>;
impl<'a, const O: u8> FREQIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FREQIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FREQIE_A::Enabled)
    }
}
#[doc = "Field `CNRDYIE` reader - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
pub type CNRDYIE_R = crate::BitReader<CNRDYIE_A>;
#[doc = "Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<CNRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CNRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNRDYIE_A {
        match self.bits {
            false => CNRDYIE_A::Disabled,
            true => CNRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CNRDYIE_A::Enabled
    }
}
#[doc = "Field `CNRDYIE` writer - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
pub type CNRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, CNRDYIE_A, O>;
impl<'a, const O: u8> CNRDYIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::Enabled)
    }
}
#[doc = "Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type AFSDETIE_R = crate::BitReader<AFSDETIE_A>;
#[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<AFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AFSDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFSDETIE_A {
        match self.bits {
            false => AFSDETIE_A::Disabled,
            true => AFSDETIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFSDETIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFSDETIE_A::Enabled
    }
}
#[doc = "Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type AFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, AFSDETIE_A, O>;
impl<'a, const O: u8> AFSDETIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::Enabled)
    }
}
#[doc = "Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type LFSDETIE_R = crate::BitReader<LFSDETIE_A>;
#[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<LFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LFSDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFSDETIE_A {
        match self.bits {
            false => LFSDETIE_A::Disabled,
            true => LFSDETIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFSDETIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFSDETIE_A::Enabled
    }
}
#[doc = "Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type LFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AIM_SPEC, LFSDETIE_A, O>;
impl<'a, const O: u8> LFSDETIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<0> {
        OVRUDRIE_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<1> {
        MUTEDETIE_W::new(self)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<2> {
        WCKCFGIE_W::new(self)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<3> {
        FREQIE_W::new(self)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<4> {
        CNRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<5> {
        AFSDETIE_W::new(self)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
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
#[doc = "SAI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aim](index.html) module"]
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
